//! Drivnod, Carnage Dominus — `{3}{B}{B}` 8/3 Legendary Phyrexian Horror.
//!
//! * "If a creature dying causes a triggered ability of a permanent you
//!   control to trigger, that ability triggers an additional time." —
//!   a static trigger-doubling replacement; not a triggered/activated
//!   ability and not expressible — GAP'd.
//! * `{B/P}{B/P}, Exile three creature cards from your graveyard:` Put
//!   an indestructible counter on Drivnod.
//!
//! The activated ability's mana cost ({B/P}{B/P}) and effect (a Named
//! "indestructible" counter on this creature) are expressed; the
//! "Exile three creature cards from your graveyard" cost has no
//! demonstrated exile-from-graveyard cost field, so that cost component
//! is a documented GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drivnod, Carnage Dominus");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let _indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP (static): "If a creature dying causes a triggered ability of a permanent you control
    // to trigger, that ability triggers an additional time" — trigger-doubling replacement, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B/P}{B/P}, Exile three creature cards from your graveyard: Put an indestructible counter on Drivnod.".into(),
            // GAP (cost): "Exile three creature cards from your graveyard" has no exile-from-graveyard cost field.
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B/P}{B/P}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_indestructible_counter,
        }),
    )
}

fn add_indestructible_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("indestructible").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind,
        count: 1,
    }]
}
