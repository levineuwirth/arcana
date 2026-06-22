//! The Eighth Doctor — `{4}{W}{U}` 4/4 Legendary Creature — Time Lord
//! Doctor.
//! "When The Eighth Doctor enters, mill three cards."
//! "Once during each of your turns, you may play a historic land or
//! cast a historic permanent spell from your graveyard. If you do, it
//! gains 'If this permanent would leave the battlefield, exile it
//! instead of putting it anywhere else.'"
//!
//! The ETB mill-three is wired. ("Mill" appears in the Scryfall
//! keyword line but is a keyword *action*, not a KeywordAbility
//! variant — it's expressed by the trigger, not the keyword vec.) The
//! once-per-turn play-from-graveyard permission is a static rules
//! modifier and is GAP'd — no play-permission primitive is expressible.

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
    let name = reg.interner_mut().intern("The Eighth Doctor");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let doctor = reg.interner_mut().intern("Doctor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(doctor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Once during each of your turns, you may play a historic land
    // or cast a historic permanent spell from your graveyard …" — a
    // static play-from-graveyard permission rules modifier isn't
    // expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_mill_three,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_mill_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill {
        player: trig.controller,
        count: 3,
    }]
}
