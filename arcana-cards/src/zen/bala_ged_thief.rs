//! Bala Ged Thief — `{3}{B}` 2/2 black Human Rogue Ally.
//!
//! * Whenever this creature or another Ally you control enters, target player
//!   reveals a number of cards from their hand equal to the number of Allies
//!   you control. You choose one of them. That player discards that card.
//!
//! The rally trigger watches any Ally you control entering (this creature
//! included). The effect is expressed as a controller-chosen discard of N
//! cards from the target player's hand, where N is the number of Allies you
//! control (dynamic). The "reveal" step is informational and folds into the
//! discard; "you choose" maps to `DiscardChoice::OpponentChooses` (the
//! discarding player's opponent — i.e. this card's controller — chooses).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bala Ged Thief");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let ally_filter = script::subtype_filter(reg, "Ally")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ally_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: rally_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn rally_discard(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let n = script::count_matching(
        state,
        &script::subtype_filter(reg, "Ally").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Discard {
        player: *p,
        count: n,
        choice: DiscardChoice::OpponentChooses,
    }]
}
