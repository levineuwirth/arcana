//! Lo and Li, Twin Tutors — `{4}{B}` 2/2 Legendary Human Advisor.
//! "When Lo and Li enter, search your library for a Lesson or Noble
//!  card, reveal it, put it into your hand, then shuffle.
//!  Noble creatures you control and Lesson spells you control have
//!  lifelink."
//!
//! The ETB tutor is wired with a subtype-OR filter (Lesson | Noble).
//! The lifelink-granting static (a continuous ability affecting OTHER
//! permanents/spells) is GAP'd — no Effect/keyword-grant static for
//! "other creatures/spells you control have …" in this card class.

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
    let name = reg.interner_mut().intern("Lo and Li, Twin Tutors");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    // Pre-intern the searched subtypes so the resolver can recover them.
    let _lesson = reg.interner_mut().intern("Lesson");
    let _noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tutor,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
    // GAP: static "Noble creatures you control and Lesson spells you
    // control have lifelink" — a continuous keyword-granting static over
    // OTHER permanents/spells; not expressible in this card class.
}

fn etb_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut syms = Vec::new();
    if let Some(s) = reg.interner().lookup("Lesson") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Noble") {
        syms.push(s);
    }
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::default().with_subtypes_any(syms),
        reveal: true,
    }]
}
