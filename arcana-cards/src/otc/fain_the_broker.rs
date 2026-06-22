//! Fain, the Broker — `{2}{B}` 3/3 Legendary Human Warlock.
//!
//! "{T}, Sacrifice a creature: Put two +1/+1 counters on target creature.
//!  {T}, Remove a counter from a creature you control: Create a Treasure token.
//!  {T}, Sacrifice an artifact: Create a 2/1 white and black Inkling creature
//!  token with flying.
//!  {3}{B}: Untap Fain."
//!
//! Four activated abilities. The Scryfall "Treasure" keyword tag is a token
//! type, not a `KeywordAbility` variant, so `keywords: vec![]`.
//!
//! Ability 2's cost — "Remove a counter from a creature YOU CONTROL" — removes a
//! counter from a CHOSEN OTHER permanent; `ActivationCost` only models
//! `remove_self_counter` (this card's own counters), so the whole ability is
//! GAP'd (cost is unexpressible).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fain, the Broker");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let _inkling = reg.interner_mut().intern("Inkling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // {T}, Sacrifice a creature: Put two +1/+1 counters on target creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a creature: Put two +1/+1 counters on target creature."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_counters,
            })
            // GAP (ability): "{T}, Remove a counter from a creature you control:
            // Create a Treasure token." — the cost removes a counter from a
            // CHOSEN OTHER creature; ActivationCost only has remove_self_counter.
            // {T}, Sacrifice an artifact: Create a 2/1 W/B Inkling with flying.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice an artifact: Create a 2/1 white and black Inkling creature token with flying.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_inkling,
            })
            // {3}{B}: Untap Fain.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}: Untap Fain.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_self,
            }),
    )
}

fn add_two_counters(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

fn make_inkling(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let inkling = reg.interner().lookup("Inkling").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(inkling);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: inkling,
            colors: ColorSet::white() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}

fn untap_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
