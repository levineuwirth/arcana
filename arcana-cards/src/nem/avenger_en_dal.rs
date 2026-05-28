//! Avenger en-Dal — `{1}{W}` 1/1 white Human Spellshaper. "{2}{W}, {T}, Discard a
//! card: Exile target attacking creature. Its controller gains life equal to its
//! toughness."
//! GAP: "Discard a card (not self)" as activation cost not in ActivationCost.
//! "Gains life equal to toughness of exiled creature" requires reading toughness
//! of the target at the time of exile.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avenger en-Dal");
    let human = reg.interner_mut().intern("Human");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}, {T}, Discard a card: Exile target attacking creature. Its controller gains life equal to its toughness.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").unwrap(),
                    tap: true,
                    // GAP: "Discard a card (not self)" not in ActivationCost
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    // GAP: "attacking creature" filter not in ObjectFilter
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_and_gain_life,
            }),
    )
}

fn exile_and_gain_life(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let toughness = script::toughness_of(state, *id).max(0) as u32;
    let target_controller = script::target_controller(state, *id, ctx.controller);
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::GainLife { player: target_controller, amount: toughness },
    ]
}
