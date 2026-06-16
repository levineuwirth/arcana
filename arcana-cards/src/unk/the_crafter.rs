//! The Crafter — `{3}{R}` 3/4 Legendary Artifact Creature — Human.
//! Affinity for artifacts.
//! "When The Crafter enters the battlefield, create two colorless scrap
//! artifact tokens."
//! "{T}, Sacrifice an artifact: Create an equipment token named Bling with
//! 'Equipped creature gets +1/+0' and 'Equip {1}.'"
//!
//! Affinity is not an expressible keyword (GAP). The ETB mints two bare
//! colorless artifact tokens (faithful — they have no abilities). The
//! activated ability mints a bare Equipment token named Bling; its printed
//! "+1/+0" static and "Equip {1}" ability cannot be authored onto a token
//! definition here, so those riders are GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Crafter");
    let human = reg.interner_mut().intern("Human");
    let _scrap = reg.interner_mut().intern("Scrap");
    let _bling = reg.interner_mut().intern("Bling");
    let _equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    // GAP: "Affinity for artifacts" — cost reduction not expressible here.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_scrap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice an artifact: Create an equipment token named Bling."
                    .into(),
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
                effect: make_bling,
            }),
    )
}

fn make_scrap(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let scrap = reg.interner().lookup("Scrap").unwrap_or_default();
    let token = TokenDefinition {
        name: scrap,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: SubtypeSet::default(),
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn make_bling(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let bling = reg.interner().lookup("Bling").unwrap_or_default();
    let equipment = reg.interner().lookup("Equipment").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    // GAP: token's "Equipped creature gets +1/+0" + "Equip {1}" riders are
    // not expressible on a TokenDefinition here; the bare token is minted.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: bling,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
