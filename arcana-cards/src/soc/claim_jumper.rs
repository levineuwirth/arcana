//! Claim Jumper — `{2}{W}` 3/3 Rabbit Mercenary. Vigilance.
//! "When this creature enters, if an opponent controls more lands than you, you
//! may search your library for a Plains card and put it onto the battlefield
//! tapped. Then if an opponent controls more lands than you, repeat this process
//! once. If you search your library this way, shuffle."
//!
//! Best-effort: the ETB search-a-Plains-to-battlefield-tapped is expressed via
//! TutorToBattlefield. The "if an opponent controls more lands than you" land-
//! parity gate and the conditional repeat are not expressible.
//!
//! GAP: intervening-if — "if an opponent controls more lands than you" (no
//! cross-player land-count comparison predicate).
//! GAP: effect — the conditional "repeat this process once" second search.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::effects::KeywordAbility;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Claim Jumper");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: search_plains,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn search_plains(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let plains = script::subtype_filter(reg, "Plains");
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: plains,
        tapped: true,
    }]
}
