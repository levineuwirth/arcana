//! Charging Hooligan — `{3}{R}` 3/3 red Human Peasant.
//! "Whenever this creature attacks, it gets +1/+0 until end of turn
//! for each attacking creature. If a Rat is attacking, this creature
//! gains trample until end of turn."
//! The pump counts attacking creatures via `ObjectFilter::attacking_only()`;
//! "if a Rat is attacking" checked via `conditions::a_permanent_matches`
//! with an attacking Rat filter.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charging Hooligan");
    let _rat = reg.interner_mut().intern("Rat");
    let human = reg.interner_mut().intern("Human");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(peasant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pump_per_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_pump_per_attacker(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        trig.controller,
    ) as i32;
    // "If a Rat is attacking, this creature gains trample."
    let rat = reg.interner().lookup("Rat").expect("Rat interned during register()");
    let rat_attacking = conditions::a_permanent_matches(
        state,
        trig.controller,
        &ObjectFilter::creature().with_subtype_sym(rat).attacking_only(),
    );
    let keywords = if rat_attacking { vec![KeywordAbility::Trample] } else { vec![] };
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords,
    }]
}
