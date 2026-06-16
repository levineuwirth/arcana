//! Rootcoil Creeper — `{G}{U}` 2/2 Plant Horror.
//! "{T}: Add one mana of any color."
//! "{T}: Add two mana of any one color. Spend this mana only to cast
//!  spells from your graveyard."
//! "{G}{U}, {T}, Exile this creature: Return target card with flashback
//!  you own from exile to your hand."
//!
//! Two tap-for-mana mana abilities are wired. "Any color" is modeled as
//! colorless mana (the catalog's fidelity convention — no any-color
//! choice primitive); the graveyard-cast spend restriction on the
//! second is also a fidelity gap. The third ability is GAP'd: a target
//! card in exile filtered by "has flashback" is not expressible, and
//! there is no return-from-exile-to-hand effect.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rootcoil Creeper");
    let plant = reg.interner_mut().intern("Plant");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add two mana of any one color. Spend this mana only to cast spells from your graveyard.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two,
            }),
        // GAP: "{G}{U}, {T}, Exile this creature: Return target card with
        // flashback you own from exile to your hand." — "card with
        // flashback" filter is not expressible and there is no
        // return-from-exile-to-hand effect. Whole ability omitted.
    )
}

fn add_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP fidelity: "any color" modeled as colorless (no any-color choice).
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn add_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP fidelity: "any one color" modeled as colorless; the
    // graveyard-only spend restriction is not expressible.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); 2],
    }]
}
