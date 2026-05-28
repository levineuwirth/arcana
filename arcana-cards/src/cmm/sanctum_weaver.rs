//! Sanctum Weaver — `{1}{G}` 0/2 green Enchantment Creature — Dryad.
//! "{T}: Add X mana of any one color, where X is the number of enchantments
//! you control."
//!
//! GAP: "any one color" — color choice not available; "X = enchantments you
//! control" is dynamic. Emitting colorless mana with count; the color choice
//! is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanctum Weaver");
    let dryad = reg.interner_mut().intern("Dryad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add X mana of any one color, where X = number of enchantments you control.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_enchantment_count_mana,
            }),
    )
}

fn add_enchantment_count_mana(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
        ctx.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    // GAP: "any one color" not expressible; emitting colorless
    let mana = (0..n).map(|_| ManaUnit::plain(ManaColor::Colorless, ctx.source)).collect();
    vec![Effect::AddMana { player: ctx.controller, mana }]
}
