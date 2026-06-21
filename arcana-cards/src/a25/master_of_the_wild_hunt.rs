//! Master of the Wild Hunt — `{2}{G}{G}` 3/3 Human Shaman.
//! At the beginning of your upkeep, create a 2/2 green Wolf creature
//! token.
//! {T}: Tap all untapped Wolf creatures you control. Each Wolf tapped this
//! way deals damage equal to its power to target creature. That creature
//! deals damage equal to its power divided as its controller chooses among
//! any number of those Wolves.
//!
//! The upkeep trigger creates a 2/2 green Wolf token. The activated
//! ability taps every untapped Wolf you control and has each deal damage
//! equal to its power to the target creature (enumerated at resolution).
//! GAP: "That creature deals damage equal to its power divided as its
//! controller chooses among any number of those Wolves." — the reciprocal
//! division back to the tapped Wolves is not expressible (DealDamageDivided
//! spreads over declared targets, not a runtime-enumerated Wolf set).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Master of the Wild Hunt");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let _wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
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
                effect: make_wolf,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tap all untapped Wolf creatures you control. Each Wolf tapped this way deals damage equal to its power to target creature.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: wolf_pack,
            }),
    )
}

fn make_wolf(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let wolf = reg.interner().lookup("Wolf").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: wolf,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn wolf_pack(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let filter = script::subtype_filter(reg, "Wolf")
        .controlled_by(ControllerConstraint::You)
        .untapped_only();
    let wolves = script::ids_matching(state, &filter, ctx.controller);
    let mut effects: Vec<Effect> = Vec::new();
    for wolf in wolves {
        let pow = script::power_of(state, wolf).max(0) as u32;
        effects.push(Effect::Tap { target: wolf });
        if pow > 0 {
            effects.push(Effect::DealDamage {
                source: wolf,
                target: DamageTarget::Object(*id),
                amount: pow,
            });
        }
    }
    // GAP: "That creature deals damage equal to its power divided as its
    // controller chooses among any number of those Wolves." — reciprocal
    // division back to a runtime-enumerated Wolf set is not expressible.
    effects
}
