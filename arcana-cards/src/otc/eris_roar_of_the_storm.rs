//! Eris, Roar of the Storm — `{8}{U}{R}` Legendary 4/4 blue-red Elemental
//! Warlock.
//!
//! Rules text:
//! * This spell costs {2} less to cast for each different mana value among
//!   instant and sorcery cards in your graveyard.
//! * Flying, prowess
//! * Whenever you cast your second spell each turn, create a 4/4 red Dragon
//!   Elemental creature token with flying and prowess.
//!
//! The cost-reduction line is a static casting modifier with no demonstrated
//! Effect / static primitive → GAP'd. "Prowess" is not in the usable keyword
//! surface → keywords: vec![KeywordAbility::Flying] only (prowess GAP'd on
//! both this card and the token). The second-spell trigger fires on each
//! spell you cast and gates inside the resolver via
//! `script::spells_cast_this_turn == 2`, creating one 4/4 red Dragon
//! Elemental token with flying.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eris, Roar of the Storm");
    let elemental = reg.interner_mut().intern("Elemental");
    let warlock = reg.interner_mut().intern("Warlock");
    // Token primary subtype.
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "costs {2} less for each different mana value among instant
    //       and sorcery cards in your graveyard" — no demonstrated cost-
    //       reduction static primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_second_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_second_spell(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Gate: only on the SECOND spell cast this turn.
    let cast = script::spells_cast_this_turn(state, &ObjectFilter::default(), trig.controller);
    if cast != 2 {
        return Vec::new();
    }
    let dragon = match reg.interner().lookup("Dragon") {
        Some(d) => d,
        None => return Vec::new(),
    };
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(dragon);
    if let Some(elemental) = reg.interner().lookup("Elemental") {
        token_subtypes.0.insert(elemental);
    }
    // GAP: the token's prowess — Prowess is not an available KeywordAbility.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dragon,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
