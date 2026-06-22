//! Lord of the Nazgûl — `{3}{U}{B}` 4/3 Legendary Wraith Noble.
//! "Flying
//!  Wraiths you control have protection from Ring-bearers.
//!  Whenever you cast an instant or sorcery spell, create a 3/3 black
//!  Wraith creature token with menace. Then if you control nine or more
//!  Wraiths, Wraiths you control have base power and toughness 9/9 until
//!  end of turn."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lord of the Nazgûl");
    let wraith = reg.interner_mut().intern("Wraith");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wraith);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: static "Wraiths you control have protection from Ring-bearers"
    //      — protection-granting to other permanents has no expressible
    //      Effect on this card class.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                ),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: make_wraith_then_buff,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_wraith_then_buff(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wraith = reg
        .interner()
        .lookup("Wraith")
        .expect("Wraith interned during register()");
    let mut tok_subtypes = SubtypeSet::default();
    tok_subtypes.0.insert(wraith);
    let token = TokenDefinition {
        name: wraith,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: tok_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        abilities: vec![],
    };
    let mut effects = vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }];
    // "Then if you control nine or more Wraiths, Wraiths you control have
    //  base power and toughness 9/9 until end of turn." (The just-created
    //  token isn't counted yet — it enters as this effect resolves.)
    let wraith_filter =
        script::subtype_filter(reg, "Wraith").controlled_by(ControllerConstraint::You);
    if script::count_matching(state, &wraith_filter, trig.controller) >= 9 {
        let ids = script::ids_matching(state, &wraith_filter, trig.controller);
        effects.push(Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::SetBasePT {
                target: NULL_OBJECT_ID,
                power: 9,
                toughness: 9,
                duration: Duration::EndOfTurn,
            }),
        });
    }
    effects
}
