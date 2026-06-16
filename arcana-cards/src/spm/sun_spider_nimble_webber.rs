//! Sun-Spider, Nimble Webber — `{3}{W/U}` 3/2 Legendary Spider Human
//! Hero. During your turn, Sun-Spider has flying (static — GAP).
//! When Sun-Spider enters, search your library for an Aura or
//! Equipment card, reveal it, put it into your hand, then shuffle.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sun-Spider, Nimble Webber");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "During your turn, Sun-Spider has flying." — a turn-gated
    // static self-keyword grant, not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tutor_aura_or_equipment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_tutor_aura_or_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let aura = reg.interner().lookup("Aura");
    let equipment = reg.interner().lookup("Equipment");
    let mut syms = Vec::new();
    if let Some(a) = aura {
        syms.push(a);
    }
    if let Some(e) = equipment {
        syms.push(e);
    }
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::default().with_subtypes_any(syms),
        reveal: true,
    }]
}
