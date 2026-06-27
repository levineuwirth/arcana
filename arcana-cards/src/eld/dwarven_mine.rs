//! Dwarven Mine — land — Mountain.
//! "({T}: Add {R}.)" / "This land enters tapped unless you control three or
//! more other Mountains." / "When this land enters untapped, create a 1/1
//! red Dwarf creature token." The conditional enters-tapped clause is wired
//! via `EntersWithSpec::TappedUnlessControlCount` over a Mountain-subtype
//! filter with `3..`; the ETB trigger fires only when it enters UNTAPPED
//! (`TriggerCondition::SelfEntersBattlefieldUntapped`).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Mine");
    let mountain = reg.interner_mut().intern("Mountain");
    let _dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mountain);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "Enters tapped unless you control three or more other Mountains."
            .with_enters_with(EntersWithSpec::TappedUnlessControlCount {
                filter: ObjectFilter::permanent().with_subtype_sym(mountain),
                min: 3,
                max: u32::MAX,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_mana,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefieldUntapped,
                intervening_if: None,
                effect: etb_dwarf_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_red_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn etb_dwarf_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Gated by SelfEntersBattlefieldUntapped — fires only when this land
    // entered untapped (the trigger condition captures that at entry).
    let dwarf = reg.interner().lookup("Dwarf").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(d) = reg.interner().lookup("Dwarf") {
        subtypes.0.insert(d);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dwarf,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
