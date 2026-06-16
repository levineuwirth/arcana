//! Demon of Fate's Design — `{4}{B}{B}` 6/6 Enchantment Creature — Demon with Flying, Trample.
//!
//! * Flying, Trample (keywords).
//! * "Once during each of your turns, you may cast an enchantment spell by
//!   paying life equal to its mana value rather than its mana cost." —
//!   alternative-cost casting permission; no primitive. GAP'd.
//! * "{2}{B}, Sacrifice another enchantment: This creature gets +X/+0 until end
//!   of turn, where X is the sacrificed enchantment's mana value." — the cost
//!   (mana + sacrifice another enchantment) is faithful; X depends on the
//!   sacrificed object's mana value, which the activated effect fn cannot read,
//!   so the dynamic pump body is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Demon of Fate's Design");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    let enchantment_filter = ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into());

    // GAP: "cast an enchantment spell by paying life equal to its mana value"
    // alternative-cost permission — no primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, Sacrifice another enchantment: This creature gets +X/+0 until end of turn, where X is the sacrificed enchantment's mana value.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    sacrifice_other: Some(enchantment_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_by_sacrificed_mv,
            }),
    )
}

fn pump_by_sacrificed_mv(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: X = sacrificed enchantment's mana value; the activation effect fn has
    // no accessor for the object paid as the sacrifice_other cost.
    Vec::new()
}
