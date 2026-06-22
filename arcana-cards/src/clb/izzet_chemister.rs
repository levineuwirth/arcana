//! Izzet Chemister — `{2}{R}` 1/3 Creature — Goblin Wizard.
//! Haste.
//! {R}, {T}: Exile target instant or sorcery card from your graveyard.
//! {1}{R}, {T}, Sacrifice this creature: Cast any number of cards exiled with
//! this creature without paying their mana costs.

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Izzet Chemister");
    let goblin = reg.interner_mut().intern("Goblin");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // {R}, {T}: Exile target instant or sorcery from your graveyard.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, {T}: Exile target instant or sorcery card from your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_from_graveyard,
            })
            // {1}{R}, {T}, Sacrifice this: free-cast exiled cards.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}, {T}, Sacrifice this creature: Cast any number of cards exiled with this creature without paying their mana costs.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: free_cast_exiled,
            }),
    )
}

fn exile_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ExileFromGraveyard { target: *id }]
}

fn free_cast_exiled(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "cast any number of cards exiled with this creature without paying
    // their mana costs" — no primitive tracks cards exiled by a specific
    // source nor free-casts them; only the {1}{R}, {T}, Sacrifice cost is
    // modeled.
    Vec::new()
}
