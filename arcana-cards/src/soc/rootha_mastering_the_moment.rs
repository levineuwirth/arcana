//! Rootha, Mastering the Moment — `{2}{U}{R}` 3/4 blue/red Legendary Orc Sorcerer.
//! "At the beginning of combat on your turn, if you've cast an instant or sorcery spell
//! this turn, create an X/X blue and red Elemental creature token with flying and haste,
//! where X is the greatest mana value among instant and sorcery spells you've cast this turn."
//!
//! GAP: "greatest mana value among instant and sorcery spells cast this turn" is not
//! expressible with the available script helpers (only spells_cast_this_turn count is
//! accessible, not max mana value); using the count as a proxy for X (incorrect).
//! The intervening-if "if you've cast an instant or sorcery spell this turn" is wired via
//! `conditions::you_cast_matching_this_turn`.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rootha, Mastering the Moment");
    let orc = reg.interner_mut().intern("Orc");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(sorcerer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
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
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                // "if you've cast an instant or sorcery spell this turn"
                intervening_if: Some(iif_cast_instant_or_sorcery_this_turn),
                effect: create_elemental_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_cast_instant_or_sorcery_this_turn(
    state: &GameState,
    _source: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_cast_matching_this_turn(
        state,
        you,
        &ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
    )
}

fn create_elemental_token(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let n = script::spells_cast_this_turn(state, &filter, trig.controller);
    if n == 0 {
        // Belt-and-suspenders alongside the intervening_if (re-checked at resolution).
        return Vec::new();
    }
    // GAP: "greatest mana value among instant and sorcery spells cast this turn" is not
    // accessible via script helpers; using count as a proxy (incorrect — should be max MV).
    let x = n as i32;
    let elemental = reg
        .interner()
        .lookup("Elemental")
        .expect("Elemental interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(x)),
        toughness: Some(PtValue::Fixed(x)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
