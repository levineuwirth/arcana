//! Oildeep Gearhulk — `{U}{U}{B}{B}` 4/4 Artifact Creature — Construct.
//!
//! Oracle:
//! * Lifelink, ward {1}.
//! * When this creature enters, look at target player's hand. You may choose a
//!   card from it. If you do, that player discards that card, then draws a card.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oildeep Gearhulk");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{B}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Lifelink,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_discard_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn etb_discard_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "look at target player's hand. You may choose a card from it. If you do,
    // that player discards that card, then draws a card." We model the
    // controller-chosen discard via OpponentChooses (the chooser is the player
    // adjacent to the discarding one, i.e. our controller in a 2-player game),
    // then the draw. Fidelity gap: the "look" + the "you may / if you do"
    // optionality are not modeled (discard is mandatory).
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Sequence(vec![
        Effect::Discard {
            player: *p,
            count: 1,
            choice: DiscardChoice::OpponentChooses,
        },
        Effect::DrawCards { player: *p, count: 1 },
    ])]
}
