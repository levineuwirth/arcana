//! Tymaret, Chosen from Death — `{B}{B}` Legendary Enchantment Creature — Demigod, 2/*.
//!
//! Oracle:
//! * Tymaret's toughness is equal to your devotion to black. (A characteristic-
//!   defining ability / variable toughness — not expressible as a fixed
//!   `PtValue`; see GAP below.)
//! * `{1}{B}: Exile up to two target cards from graveyards. You gain 1 life for
//!   each creature card exiled this way.`
//!
//! The activated ability's exile is expressed; the "gain 1 life for each
//! creature card exiled this way" rider depends on inspecting what was exiled
//! at resolution and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tymaret, Chosen from Death");
    let demigod = reg.interner_mut().intern("Demigod");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demigod);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        // GAP: toughness is "*", equal to your devotion to black (a CDA / variable
        // toughness). No PtValue variant expresses a dynamic toughness; using a
        // placeholder fixed value.
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{B}: Exile up to two target cards from graveyards. You gain 1 life for each creature card exiled this way.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::default(),
                },
                count: TargetCount::UpTo(2),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: exile_graveyard_cards,
        }),
    )
}

fn exile_graveyard_cards(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Exile up to two chosen graveyard cards.
    let mut effects: Vec<Effect> = Vec::new();
    for target in &ctx.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ExileFromGraveyard { target: *id });
        }
    }
    // GAP: "You gain 1 life for each creature card exiled this way" — the life
    // amount depends on how many of the exiled cards were creatures, which the
    // effect cannot inspect after the fact with the available API.
    effects
}
