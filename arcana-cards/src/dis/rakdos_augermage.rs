//! Rakdos Augermage — `{B}{B}{R}` 3/2 Human Wizard with First strike.
//! "{T}: Reveal your hand and discard a card of target opponent's choice.
//! Then that player reveals their hand and discards a card of your choice.
//! Activate only as a sorcery."

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakdos Augermage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Reveal your hand and discard a card of target opponent's choice. Then that player reveals their hand and discards a card of your choice. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: mutual_discard,
        }),
    )
}

fn mutual_discard(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(opp) = target else {
        return Vec::new();
    };
    // Two sequential player-directed discard choices. They MUST be wrapped in
    // an Effect::Sequence so the resolution park loop flattens them and parks
    // between the two choices — returning them as a flat top-level Vec posts
    // the second pending choice while the first is still pending and trips the
    // single-slot invariant (state.rs push_pending_choice).
    vec![Effect::Sequence(vec![
        // You discard a card of the target opponent's choice.
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::OpponentChooses,
        },
        // That player discards a card of your choice.
        Effect::Discard {
            player: *opp,
            count: 1,
            choice: DiscardChoice::OpponentChooses,
        },
    ])]
}
