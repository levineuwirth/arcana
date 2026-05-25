//! Eclipsed Boggart — `{B/R}{B/R}{B/R}` 2/3 black-red Goblin Scout.
//! "When this creature enters, look at the top four cards of your
//! library. You may reveal a Goblin, Swamp, or Mountain card from
//! among them and put it into your hand. Put the rest on the bottom
//! of your library in a random order."
//! GAP: selective reveal/hand from among top 4 with subtype/land filter
//! — not expressible; Surveil(4) used as approximation.

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
    let name = reg.interner_mut().intern("Eclipsed Boggart");
    let goblin = reg.interner_mut().intern("Goblin");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B/R}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_look_top_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_look_top_four(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: look at top 4, reveal Goblin/Swamp/Mountain to hand, rest bottom
    // — selective reveal not in engine catalog; Surveil(4) as approximation
    vec![Effect::Surveil { player: trig.controller, count: 4 }]
}
