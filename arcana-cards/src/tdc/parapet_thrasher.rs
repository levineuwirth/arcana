//! Parapet Thrasher — `{2}{R}{R}` 4/3 Creature — Dragon.
//!
//! * Flying.
//! * Whenever one or more Dragons you control deal combat damage to an
//!   opponent, choose one that hasn't been chosen this turn —
//!   • Destroy target artifact that opponent controls.
//!   • This creature deals 4 damage to each other opponent.
//!   • Exile the top card of your library. You may play it this turn.
//!
//! Modal TRIGGERED abilities aren't supported (modal is a spell-ability
//! surface). The trigger fires on Dragons-you-control combat damage to an
//! opponent and is wired to the third mode (impulse-exile the top card).
//! GAP: the modal choice and the per-turn "not chosen this turn" constraint
//!   aren't expressible on a trigger.
//! GAP: modes 1 (destroy target artifact) and 2 (4 damage to each OTHER
//!   opponent) are omitted in favor of the clean no-target mode 3.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Parapet Thrasher");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let dragons_you_control = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: dragons_you_control,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: impulse_top_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn impulse_top_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}
