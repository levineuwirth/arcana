//! Arabella, Abandoned Doll — `{R}{W}` 1/3 Legendary Artifact Creature — Toy.
//! "Whenever Arabella attacks, it deals X damage to each opponent and you gain X life, where X is the number of creatures you control with power 2 or less."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::script;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arabella, Abandoned Doll");
    let toy_sub = reg.interner_mut().intern("Toy");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(toy_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: arabella_abandoned_doll_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn arabella_abandoned_doll_trigger(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(state, &ObjectFilter::creature().with_max_power(2).controlled_by(ControllerConstraint::You), trig.controller);
    if n == 0 { return Vec::new(); }
    let opponents = script::opponents(state, trig.controller);
    let mut effects: Vec<Effect> = opponents.into_iter()
        .map(|p| Effect::DealDamage { target: DamageTarget::Player(p), amount: n, source: trig.source })
        .collect();
    effects.push(Effect::GainLife { player: trig.controller, amount: n });
    effects
}
