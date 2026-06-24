//! Orcish Settlers — `{1}{R}` 1/1 red Orc.
//! `{X}{X}{R}, {T}, Sacrifice this creature: Destroy X target lands.`
//! Wired as an `{X}{X}{R}, {T}, Sacrifice` activated ability — the engine
//! charges 2·X (both `{X}` symbols expand to the chosen X) and `TargetCount::X`
//! selects X target lands, each destroyed.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orcish Settlers");
    let orc = reg.interner_mut().intern("Orc");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{X}{R}, {T}, Sacrifice this creature: Destroy X target lands.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{X}{R}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::X,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_land,
            }),
    )
}

fn destroy_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::DestroyPermanent { target: *id }),
            _ => None,
        })
        .collect()
}
