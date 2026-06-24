//! Bonny Pall, Clearcutter — `{3}{G}{U}{U}` 6/5 Legendary Creature — Giant
//! Scout with Reach.
//!
//! Reach.
//! When Bonny Pall enters, create Beau, a legendary blue Ox creature token
//! with "Beau's power and toughness are each equal to the number of lands you
//! control."
//! Whenever you attack, draw a card, then you may put a land card from your
//! hand or graveyard onto the battlefield.
//!
//! Reach is a base keyword. The ETB mints the Beau token; Beau's */* CDA
//! ("equal to the number of lands you control") is represented as PtValue::Star.
//! The lands-count CDA itself is GAP'd: it belongs to the freshly-minted TOKEN,
//! not to Bonny Pall, so the source-anchored `self_pt_*` install (which targets
//! `trig.source`) cannot reach it; and the only token-borne route,
//! `TokenDefinition.abilities`, is dropped by `mint_one_token` (the minted
//! GameObject is built from `to_characteristics()` only, so a token-carried
//! SelfEntersBattlefield CDA install would never fire). The
//! attack trigger draws a card and offers a land put from HAND (the graveyard
//! source is a partial — only the hand path is expressible via
//! PutFromHandOntoBattlefield).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bonny Pall, Clearcutter");
    let giant = reg.interner_mut().intern("Giant");
    let scout = reg.interner_mut().intern("Scout");
    let _ox = reg.interner_mut().intern("Ox");
    let _beau = reg.interner_mut().intern("Beau");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_beau,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_attack_draw_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_beau(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let beau = reg.interner().lookup("Beau").unwrap_or_default();
    let ox = reg.interner().lookup("Ox").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ox);
    let token = TokenDefinition {
        name: beau,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Beau's P/T CDA "equal to the number of lands you control" is a
        // token characteristic-defining ability; unmodeled. Represented as */*.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn on_attack_draw_land(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        // Partial: only the HAND source of the optional land put is expressible.
        // GAP: the "or graveyard" source is not covered by PutFromHandOntoBattlefield.
        Effect::PutFromHandOntoBattlefield {
            player: trig.controller,
            filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            tapped: false,
        },
    ]
}
