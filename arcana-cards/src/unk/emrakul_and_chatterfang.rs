//! Emrakul and Chatterfang — `{8}{G}{G}{G}` 15/15 Legendary Eldrazi Squirrel.
//! * Devoid — the card is colorless (modeled via ColorSet::colorless()).
//! * Flying, Trample, Forestwalk — base keywords (Forestwalk →
//!   Landwalk(Forest)).
//! * "protection from non-Squirrels" — Protection is not in the keyword
//!   surface; GAP.
//! * "When you cast Emrakul and Chatterfang, create 15 1/1 green Eldrazi
//!   Squirrel Scion creature tokens with 'Sacrifice this creature: Add
//!   {G} or {B}'." The 15 tokens are minted; the token's sacrifice-for-
//!   mana activated ability is GAP'd (no token-ability authoring here).

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
    let name = reg.interner_mut().intern("Emrakul and Chatterfang");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(squirrel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(15)),
        toughness: Some(PtValue::Fixed(15)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Trample,
            KeywordAbility::Landwalk(forest),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter { name: Some(name), ..ObjectFilter::default() }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_scions,
                trigger_zones: vec![Zone::Stack],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_scions(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let eldrazi = reg.interner().lookup("Eldrazi").unwrap_or_default();
    let squirrel = reg.interner().lookup("Squirrel").unwrap_or_default();
    let scion = reg.interner().lookup("Scion").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(squirrel);
    subtypes.0.insert(scion);
    // GAP: token's "Sacrifice this creature: Add {G} or {B}" mana ability
    // is not authored on the TokenDefinition.
    let token = TokenDefinition {
        name: scion,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }; 15]
}
