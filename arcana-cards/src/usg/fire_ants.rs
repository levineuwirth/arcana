//! Fire Ants — `{2}{R}` 2/1 red Insect.
//! "{T}: This creature deals 1 damage to each other creature without flying."
//! GAP: "without flying" — ObjectFilter has no "without keyword" method;
//! using all creatures as approximation (will hit flying creatures too).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire Ants");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature deals 1 damage to each other creature without flying.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_creatures_no_flying,
            }),
    )
}

fn damage_creatures_no_flying(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "without flying" — using all creatures as approximation.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    let ids_excl_self: Vec<_> = ids.into_iter().filter(|&id| id != ctx.source).collect();
    vec![Effect::ForEach {
        targets: ids_excl_self,
        effect: Box::new(Effect::DealDamage {
            target: DamageTarget::Object(arcana_core::objects::NULL_OBJECT_ID),
            amount: 1,
            source: ctx.source,
        }),
    }]
}
