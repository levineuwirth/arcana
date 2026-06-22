//! Shilgengar, Sire of Famine — `{3}{B}{B}` 6/6 Legendary Creature —
//! Elder Demon.
//! Flying.
//! Sacrifice another creature: Create a Blood token. If you sacrificed an
//! Angel this way, create a number of Blood tokens equal to its toughness
//! instead.
//! {W/B}{W/B}{W/B}, Sacrifice six Blood tokens: Return each creature card
//! from your graveyard to the battlefield with a finality counter on it.
//! Those creatures are Vampires in addition to their other types.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shilgengar, Sire of Famine");
    let elder = reg.interner_mut().intern("Elder");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(demon);
    let _blood = reg.interner_mut().intern("Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    let blood_filter = script::subtype_filter(reg, "Blood");
    reg.register(
        CardDefinition::new(name, chars)
            // Sacrifice another creature: Create a Blood token.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another creature: Create a Blood token.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
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
                effect: make_blood,
            })
            // {W/B}{W/B}{W/B}, Sacrifice six Blood: mass reanimation.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W/B}{W/B}{W/B}, Sacrifice six Blood tokens: Return each creature card from your graveyard to the battlefield with a finality counter on it. Those creatures are Vampires in addition to their other types.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W/B}{W/B}{W/B}").expect("valid cost"),
                    sacrifice_other: Some(blood_filter),
                    sacrifice_other_count: 6,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mass_reanimate,
            }),
    )
}

fn make_blood(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If you sacrificed an Angel this way, create Blood equal to its
    // toughness instead" — the sacrificed permanent isn't readable in the
    // effect fn (the engine pays the sacrifice_other cost automatically); a
    // single Blood token is always created.
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Blood,
        count: 1,
    }]
}

fn mass_reanimate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return EACH creature card" (returns one chosen creature instead —
    // no all-graveyard reanimation primitive), the finality counter, and the
    // "those creatures are Vampires" type-grant rider are not expressible.
    vec![Effect::Reanimate {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        from_zone: Zone::Graveyard(ctx.controller),
    }]
}
