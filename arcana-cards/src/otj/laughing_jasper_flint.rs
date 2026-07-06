//! Laughing Jasper Flint — `{1}{B}{R}` 4/3 Legendary Lizard Rogue.
//! "Creatures you control but don't own are Mercenaries in addition to
//! their other types."
//! "At the beginning of your upkeep, exile the top X cards of target
//! opponent's library, where X is the number of outlaws you control.
//! Until end of turn, you may cast spells from among those cards, and
//! mana of any type can be spent to cast those spells."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Laughing Jasper Flint");
    let lizard = reg.interner_mut().intern("Lizard");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(rogue);
    // GAP (static): "Creatures you control but don't own are Mercenaries in
    // addition to their other types" — a controlled-but-not-owned type-adding
    // static isn't expressible in this card class.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_exile_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_opponent()],
            }),
    )
}

fn upkeep_exile_opponent(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top X cards of TARGET OPPONENT's library, you may cast
    // them this turn with any mana." ImpulseExile only acts on the controller's
    // own library and has no per-opponent / any-type-mana variant; the whole
    // effect is inexpressible.
    Vec::new()
}
