//! Eiganjo Exemplar — `{1}{W}` 2/1 white Enchantment Creature — Human Samurai.
//! "Whenever a Samurai or Warrior you control attacks alone, it gets +1/+1 until end of turn."
//!
//! GAP: "attacks alone" filter and "a Samurai or Warrior" source filter are not available
//! in CreatureAttacks. Using CreatureAttacks with Warrior subtype filter as best-effort.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eiganjo Exemplar");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);
    let warrior_filter = script::subtype_filter(reg, "Warrior")
        .controlled_by(ControllerConstraint::You);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "Samurai or Warrior attacks alone" — using Warrior attacks (you control).
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: warrior_filter,
                },
                intervening_if: None,
                effect: on_warrior_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_warrior_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.entering_object().unwrap_or(trig.source);
    vec![Effect::Pump {
        target: id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
