//! Cankerbloom — `{1}{G}` 3/2 Phyrexian Fungus.
//! `{1}, Sacrifice this creature: Choose one —`
//! • Destroy target artifact.
//! • Destroy target enchantment.
//! • Proliferate.
//!
//! Activated abilities have no modal field in the engine API, so the
//! "choose one" cannot be expressed on an activated ability. The cost
//! ({1}, sacrifice self) is faithful; the modal effect body is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cankerbloom");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let fungus = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(fungus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice this creature: Choose one — Destroy target artifact. • Destroy target enchantment. • Proliferate.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: modal_choice,
            }),
    )
}

// GAP: modal "Choose one" (destroy target artifact / destroy target enchantment /
// proliferate) is not expressible on an activated ability — ActivatedAbilityDef has
// no `modal` field, and the three modes carry different targets. Cost is faithful.
fn modal_choice(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
