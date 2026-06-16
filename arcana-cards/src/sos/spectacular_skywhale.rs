//! Spectacular Skywhale — `{2}{U}{R}` 1/4 Elemental Whale with Flying.
//! Opus — Whenever you cast an instant or sorcery spell, it gets +3/+0 EOT
//! (the "if five or more mana was spent → three +1/+1 counters instead"
//! rider is unexpressible — see GAP below).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spectacular Skywhale");
    let elemental = reg.interner_mut().intern("Elemental");
    let whale = reg.interner_mut().intern("Whale");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(whale);

    // Opus is not an expressible KeywordAbility — it's just a label on the
    // triggered ability below, so keywords carries only Flying.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(arcana_core::targets::ObjectFilter::new().with_types_any(
                    TypeLine(TypeLine::INSTANT | TypeLine::SORCERY),
                )),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: on_cast_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_cast_pump(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "If five or more mana was spent to cast that spell, put three +1/+1
    // counters on this creature instead" — the mana-spent amount of the
    // triggering spell is not available to the effect fn, so the conditional
    // counter branch is omitted; we always apply the +3/+0 base.
    vec![Effect::Pump {
        target: trig.source,
        power: 3,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
