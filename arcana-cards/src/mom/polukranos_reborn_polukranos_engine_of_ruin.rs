//! Polukranos Reborn // Polukranos, Engine of Ruin — `{G}{G}{G}` green Legendary Hydra 4/5 (front) /
//! Legendary Phyrexian Hydra (back). Transform card.
//!
//! Front face:
//!   Reach
//!   {6}{W/P}: Transform Polukranos Reborn. Activate only as a sorcery.
//!
//! Back face (Polukranos, Engine of Ruin):
//!   Reach, lifelink
//!   Whenever Polukranos or another nontoken Hydra you control dies, create a 3/3 green
//!   and white Phyrexian Hydra creature token with reach and a 3/3 green and white
//!   Phyrexian Hydra creature token with lifelink.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Polukranos Reborn");
    let hydra_sub = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Reach],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // Back face: Polukranos, Engine of Ruin — Legendary Phyrexian Hydra
    let back_name = reg.interner_mut().intern("Polukranos, Engine of Ruin");
    let back_hydra_sub = reg.interner_mut().intern("Hydra");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_hydra_sub);
    back_subtypes.0.insert(phyrexian_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Reach, KeywordAbility::Lifelink],
            power: Some(PtValue::Fixed(8)),
            toughness: Some(PtValue::Fixed(8)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Hoisted before reg.register(...) to avoid borrowing reg immutably inside the
    // &mut reg receiver expression. back_hydra_sub == this symbol; reuse it directly.
    let hydra_sym = back_hydra_sub;

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Activated ability: {6}{W/P}: Transform. Sorcery speed (face 0 only).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{W/P}: Transform Polukranos Reborn. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{W/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false, // sorcery speed
                face_gate: Some(0),      // front face only
                effect: transform_self,
            })
            // Back face: "Whenever Polukranos or another nontoken Hydra you control
            // dies, create a 3/3 green and white Phyrexian Hydra with reach and a 3/3
            // green and white Phyrexian Hydra with lifelink." Gated to the back face.
            // The filter matches nontoken Hydra creatures you control; Polukranos itself
            // is nontoken, so "Polukranos or another" is covered by the single filter.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .nontoken()
                        .controlled_by(ControllerConstraint::You)
                        .with_subtype_sym(hydra_sym),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: create_hydra_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1), // back face only
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

/// Create two 3/3 green-and-white Phyrexian Hydra tokens: one with reach, one with lifelink.
fn create_hydra_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let hydra = reg.interner().lookup("Hydra").expect("Hydra interned during register()");
    let phyrexian = reg.interner().lookup("Phyrexian").expect("Phyrexian interned during register()");
    let token_name = reg.interner().lookup("Polukranos, Engine of Ruin")
        .expect("back-face name interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(hydra);
    token_subtypes.0.insert(phyrexian);
    let base = |keyword: KeywordAbility| TokenDefinition {
        name: token_name,
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes.clone(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![keyword],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: base(KeywordAbility::Reach),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: base(KeywordAbility::Lifelink),
        },
    ]
}
