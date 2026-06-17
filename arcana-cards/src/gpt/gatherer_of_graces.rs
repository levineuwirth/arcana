//! Gatherer of Graces — `{1}{G}` 1/2 Creature — Human Druid (green).
//!
//! * "This creature gets +1/+1 for each Aura attached to it." — a power/
//!   toughness-defining static (CDA); not expressible via the documented
//!   effect surface. GAP'd.
//! * "Sacrifice an Aura: Regenerate this creature." — activated ability whose
//!   cost is sacrificing a chosen Aura you control; regenerates this creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gatherer of Graces");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    // GAP: "+1/+1 for each Aura attached to it" is a defining static.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let aura_filter = ObjectFilter::new()
        .with_types(TypeLine::ENCHANTMENT.into())
        .with_subtype_sym(aura);

    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
        text: "Sacrifice an Aura: Regenerate this creature.".into(),
        cost: ActivationCost {
            sacrifice_other: Some(aura_filter),
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: regenerate_self,
    }))
}

fn regenerate_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
