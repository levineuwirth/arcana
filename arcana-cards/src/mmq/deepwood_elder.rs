//! Deepwood Elder — `{G}{G}` 2/2 Creature — Dryad Spellshaper.
//! `{X}{G}{G}, {T}, Discard a card: X target lands become Forests until end of turn.`
//! GAP: "become Forests until end of turn" — no Effect variant for land type-changing.
//! GAP: X-target lands (TargetCount::X from x_value, but land type-change not implementable).

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
    let name = reg.interner_mut().intern("Deepwood Elder");
    let dryad = reg.interner_mut().intern("Dryad");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{G}{G}, {T}, Discard a card: X target lands become Forests until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{G}{G}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_forests,
            }),
    )
}

fn make_forests(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant for "land becomes Forest until end of turn" (subtype/type layer change)
    Vec::new()
}
