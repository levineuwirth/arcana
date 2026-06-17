//! Katara, Seeking Revenge — `{3}{U/B}` 3/3 Legendary Human Warrior Ally.
//! Waterbend additional cost is GAP'd (no waterbend cost field). When she
//! enters, draw a card (the "then discard unless waterbend was paid" rider
//! is GAP'd — additional-cost-paid state isn't queryable). The static
//! +1/+1-per-Lesson is GAP'd (no continuous CDA primitive).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Katara, Seeking Revenge");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    subtypes.0.insert(ally);

    // GAP: keyword Waterbend / "waterbend {2}" additional cost — no surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: enters_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn enters_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then discard a card unless her additional cost was paid" — the
    // waterbend-paid flag isn't queryable, so the conditional discard is omitted.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
